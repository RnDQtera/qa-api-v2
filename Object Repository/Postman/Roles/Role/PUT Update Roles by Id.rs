<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT Update Roles by Id</name>
   <tag></tag>
   <elementGuidId>d7592afe-04dc-4a85-94d2-07a89392b375</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${bt}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot;: \&quot;muhamad ilham mubarrok edited\&quot;,\n  \&quot;desc\&quot;: \&quot;orang\&quot;,\n  \&quot;managePermission\&quot;: {\n    \&quot;assetAttribute\&quot;: {\n      \&quot;update\&quot;: true,\n      \&quot;view\&quot;: true,\n      \&quot;delete\&quot;: false,\n      \&quot;create\&quot;: false\n    },\n    \&quot;assetPolicies\&quot;: {\n      \&quot;update\&quot;: true,\n      \&quot;view\&quot;: true,\n      \&quot;create\&quot;: false,\n      \&quot;delete\&quot;: false\n    },\n    \&quot;depreciationGroup\&quot;: {\n      \&quot;update\&quot;: true,\n      \&quot;create\&quot;: false,\n      \&quot;view\&quot;: true,\n      \&quot;delete\&quot;: false\n    },\n    \&quot;depreciationMethod\&quot;: {\n      \&quot;create\&quot;: false,\n      \&quot;update\&quot;: true,\n      \&quot;view\&quot;: true,\n      \&quot;delete\&quot;: false\n    },\n    \&quot;documentDeletion\&quot;: {\n      \&quot;create\&quot;: false,\n      \&quot;view\&quot;: true,\n      \&quot;delete\&quot;: false,\n      \&quot;update\&quot;: true\n    },\n    \&quot;group\&quot;: {\n      \&quot;create\&quot;: false,\n      \&quot;view\&quot;: true,\n      \&quot;update\&quot;: true,\n      \&quot;delete\&quot;: false\n    },\n    \&quot;user\&quot;: {\n      \&quot;update\&quot;: true,\n      \&quot;create\&quot;: false,\n      \&quot;view\&quot;: true,\n      \&quot;delete\&quot;: false\n    },\n    \&quot;iotReader\&quot;: {\n      \&quot;create\&quot;: false,\n      \&quot;update\&quot;: true,\n      \&quot;delete\&quot;: false,\n      \&quot;view\&quot;: true\n    },\n    \&quot;license\&quot;: {\n      \&quot;create\&quot;: false,\n      \&quot;view\&quot;: true,\n      \&quot;update\&quot;: true,\n      \&quot;delete\&quot;: false\n    },\n    \&quot;purchaseDocument\&quot;: {\n      \&quot;view\&quot;: true,\n      \&quot;create\&quot;: false,\n      \&quot;update\&quot;: true,\n      \&quot;delete\&quot;: false\n    },\n    \&quot;purchaseInformation\&quot;: {\n      \&quot;create\&quot;: false,\n      \&quot;update\&quot;: true,\n      \&quot;view\&quot;: true,\n      \&quot;delete\&quot;: false\n    },\n    \&quot;role\&quot;: {\n      \&quot;create\&quot;: false,\n      \&quot;update\&quot;: true,\n      \&quot;delete\&quot;: false,\n      \&quot;view\&quot;: true\n    },\n    \&quot;tag\&quot;: {\n      \&quot;create\&quot;: false,\n      \&quot;delete\&quot;: false,\n      \&quot;update\&quot;: true,\n      \&quot;view\&quot;: true\n    },\n    \&quot;registerAsset\&quot;: {\n      \&quot;delete\&quot;: false,\n      \&quot;create\&quot;: false,\n      \&quot;update\&quot;: true,\n      \&quot;view\&quot;: true\n    }\n  },\n  \&quot;groups\&quot;: [\n    \&quot;6458d4ea9baf734db2b39434\&quot;\n  ],\n  \&quot;isActive\&quot;: false\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>ab7af04b-e8d6-44a8-90d6-96424bae2a29</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bt}</value>
      <webElementGuid>5b4f7eb7-33f6-4148-817c-d55d087e22d3</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${rolUrl}${rol}66320e1d3488e03df890ef4e</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.ROLES_BASE_URL</defaultValue>
      <description></description>
      <id>393bef34-a31b-46dd-a976-675b75fd0452</id>
      <masked>false</masked>
      <name>rolUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.rolesEndpoint</defaultValue>
      <description></description>
      <id>7d62aa0b-2cea-4f1a-8ed3-01018d3a679b</id>
      <masked>false</masked>
      <name>rol</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Bearer</defaultValue>
      <description></description>
      <id>0a7b9c8d-fa3d-4c68-a254-68f2982573fc</id>
      <masked>false</masked>
      <name>bt</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
